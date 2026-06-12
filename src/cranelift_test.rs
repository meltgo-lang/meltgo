use cranelift_codegen::ir::types::*;
use cranelift_codegen::ir::{AbiParam, InstBuilder};
use cranelift_codegen::settings;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataDescription, Linkage, Module, default_libcall_names};
use cranelift_native::builder_with_options;
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::fs::File;
use std::io::Write;

fn main() {
    // 1. ターゲットとコード生成設定の準備
    let flag_builder = settings::builder();
    let isa_builder = builder_with_options(false).unwrap();
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();

    // 2. オブジェクトファイル出力用モジュールの作成
    let mut module =
        ObjectModule::new(ObjectBuilder::new(isa, "my_program", default_libcall_names()).unwrap());

    // 3. 外部関数 `printf` の宣言
    let mut printf_sig = module.make_signature();
    printf_sig.params.push(AbiParam::new(I64)); // フォーマット文字列へのポインタ
    printf_sig.params.push(AbiParam::new(I32)); // 足し算の結果 (i32)
    printf_sig.returns.push(AbiParam::new(I32)); // 戻り値 (i32)
    let printf_func_id = module
        .declare_function("printf", Linkage::Import, &printf_sig)
        .unwrap();

    // 4. データセクションにフォーマット文字列を配置
    let fmt_str = b"%d\n\0";
    let mut fmt_data = DataDescription::new();
    fmt_data.define(fmt_str.to_vec().into_boxed_slice());
    let fmt_id = module
        .declare_data("fmt", Linkage::Local, false, false)
        .unwrap();
    module.define_data(fmt_id, &fmt_data).unwrap();

    // 5. `main` 関数の定義
    let mut main_sig = module.make_signature();
    main_sig.returns.push(AbiParam::new(I32));
    let main_func_id = module
        .declare_function("main", Linkage::Export, &main_sig)
        .unwrap();

    let mut ctx = module.make_context();
    ctx.func.signature = main_sig.clone();

    let mut builder_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

    let block = builder.create_block();
    builder.switch_to_block(block);
    builder.seal_block(block);

    // 足し算: 2 + 3
    let val1 = builder.ins().iconst(I32, 2);
    let val2 = builder.ins().iconst(I32, 3);
    let sum = builder.ins().iadd(val1, val2);

    // printf の呼び出し準備
    let fmt_ptr = module.declare_data_in_func(fmt_id, builder.func);
    let fmt_val = builder.ins().global_value(I64, fmt_ptr);

    // 関数呼び出し
    let call_args = vec![fmt_val, sum];
    let printf_func_ref = module.declare_func_in_func(printf_func_id, &mut builder.func);
    builder.ins().call(printf_func_ref, &call_args);

    // return 0
    let zero = builder.ins().iconst(I32, 0);
    builder.ins().return_(&[zero]);

    builder.finalize();

    // 6. 関数をモジュールに定義
    module.define_function(main_func_id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);

    // 7. オブジェクトファイルを生成して書き出し
    let product = module.finish();
    let obj = product.object;
    let bytes = obj.write().unwrap();

    let mut file = File::create("output.obj").unwrap();
    file.write_all(&bytes).unwrap();

    println!("コンパイル完了: output.o");
    println!("以下のコマンドでリンクおよび実行できます:");
    println!("  $ gcc output.obj -o a.exe");
    println!("  $ ./a.exe");
}
