use melior::{
    context::Context,
    dialect::{func, arith},
    ir::{
        attribute::{StringAttribute, TypeAttribute},
        r#type::Type,
        Block, Location, Module, Region,
    },
    pass::{self, PassManager},
    execution_engine::ExecutionEngine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. MLIR コンテキストの初期化とダイアレクトの登録
    let context = Context::new();
    context.append_dialect_registry(&melior::dialect::registry());
    context.load_all_available_dialects();

    // 2. モジュールと位置情報の作成
    let location = Location::unknown(&context);
    let mut module = Module::new(location);

    // 3. 型の定義 (32ビット整数)
    let i32_type = Type::integer(&context, 32);

    // 4. 関数のリージョンとブロックの作成
    let region = Region::new();
    let block = Block::new(&[(i32_type, location), (i32_type, location)]);

    // ブロックの引数（入力となる2つの整数）を取得
    let arg1 = block.argument(0)?;
    let arg2 = block.argument(1)?;

    // 5. 足し算オペレーション (arith.addi) の追加
    let add_operation = arith::addi(arg1.into(), arg2.into(), location);
    let add_result = add_operation.result(0)?;
    block.append_operation(add_operation);

    // 6. 関数からの返り値オペレーション (func.return) の追加
    block.append_operation(func::r#return(&[add_result.into()], location));
    region.append_block(block);

    // 7. 関数全体 (func.func) を構築してモジュールに追加
    let func_type = TypeAttribute::new(Type::function(&context, &[i32_type, i32_type], &[i32_type]));
    let func_op = func::func(
        &context,
        StringAttribute::new(&context, "add_func"),
        func_type,
        region,
        &[],
        location,
    );
    module.body().append_operation(func_op);

    // 構築されたMLIRコードをテキストとして表示
    println!("--- 生成された MLIR ---");
    module.as_operation().dump();
    println!("\n----------------------");

    // 8. 実行エンジン (JIT) のためのパスのセットアップ
    // MLIRの高級な命令を、LLVM IRに変換（ロジックの共通化）するための最適化パス
    let pass_manager = PassManager::new(&context);
    pass_manager.add_pass(pass::conversion::create_convert_to_llvm());
    // 必要に応じて他の変換パスを追加（例: create_convert_func_to_llvm など）
    pass_manager.run(&mut module)?;

    // 9. 実行エンジンの作成と関数の呼び出し
    let engine = ExecutionEngine::new(&module, 2, &[], false);

    let mut arg_x: i32 = 42;
    let mut arg_y: i32 = 58;
    let mut res: i32 = 0;

    // 引数と結果のポインタを渡して実行
    unsafe {
        engine.invoke_packed(
            "add_func",
            &mut [
                &mut arg_x as *mut i32 as *mut std::ffi::c_void,
                &mut arg_y as *mut i32 as *mut std::ffi::c_void,
                &mut res as *mut i32 as *mut std::ffi::c_void,
            ],
        )?;
    }

    println!("実行結果: {} + {} = {}", arg_x, arg_y, res);

    Ok(())
}
