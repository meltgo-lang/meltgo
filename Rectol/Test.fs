namespace Test

module Sample1 =
    // 任意の深さを持つDUの例
    type A =
        | Node of A * A
        | C of string
        | Leaf of int

    /// 指定した DU を走査し、特定のケース C を引数リストの値で置換する
    let createReplacer (inputA: A) =
        // recursive helper: (現在のMap, 未使用のリスト) -> (新しいA, 更新されたMap, 残りのリスト)
        let rec transform (mapped, available) = function
            | Node (left, right) ->
                let newLeft, map1, list1 = transform (mapped, available) left
                let newRight, map2, list2 = transform (map1, list1) right
                Node(newLeft, newRight), map2, list2
            
            | C str ->
                match Map.tryFind str mapped with
                | Some existingValue -> 
                    // すでに出現した文字列なら、以前と同じ値を再利用
                    existingValue, mapped, available
                | None ->
                    // 新しい文字列なら、リストの先頭から値を取ってMapに記録
                    match available with
                    | head :: tail -> head, Map.add str head mapped, tail
                    | [] -> failwith "引数リストの要素が足りません"
                
            | Leaf i -> Leaf i, mapped, available

        // 最終的に A list -> A という関数を返す
        fun (values: A list) ->
            let result, _, _ = transform (Map.empty, values) inputA
            result

module Sample2 =
    open Psictre.TypeReserach

    type Symbol(t: string) =
        interface ITRConstrait with
            member __.Delegate (target: ITRConstrait): ITRConstrait option = 
                match target with
                | :? Symbol as source ->
                    if t = (source :> ITRConstraitGetter<string>).Get()
                    then Some target
                    else None
                | _ -> None

        interface ITRConstraitGetter<string> with
            member __.Get () = t

    type Undef(t: string) =
        interface ITRConstrait with
            member __.Delegate (target: ITRConstrait): ITRConstrait option = 
                Some target

        interface ITRConstraitGetter<string> with
            member __.Get () = t

    type Constrait(t: string, list: ITRConstrait list) = class end
