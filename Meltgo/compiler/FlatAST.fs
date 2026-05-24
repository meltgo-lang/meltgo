namespace Mycig.Compiler

open FParsec

type private NodePos = {
    Line: int64
    Column: int64
}

type ASTNode =
    | Package of NodePos * string
    | Import of NodePos * string
    | TypeFunc of NodePos * int list * int
    | Type of NodePos * int
    | If of NodePos * int * int list * (int * int list) list * int list

exception FASTParseException of string

type FASTResult<'T, 'E> =
    | Ok of 'T
    | Err of 'E
    member this.unwrap() =
        match this with
        | Ok v -> v
        | Err _ -> failwith "FASTResult.unwrap() ->! It's Not FASTRsult.Ok."

type FlatAST() =
    let ident = attempt (regex "[\w_][\w\d_]*(::[\w_][\w\d_]*)*")

    let bool_ =
        pstring "bool" .>> spaces .>> pchar ':' .>> spaces
        >>. choice [ attempt (stringReturn "true" true)
                     stringReturn "false" false ]
        .>> spaces
        |>> box

    let str_ =
        pstring "str" .>> spaces .>> pchar ':'
        >>. between (spaces .>> pchar '"') (pchar '"' .>> spaces) (manyChars (noneOf "\""))
        .>> spaces
        |>> box

    let ref_ =
        pstring "ref" .>> spaces .>> pchar ':' .>> spaces
        >>. pint32
        .>> spaces
        |>> box

    let arr_, arrRef = createParserForwardedToRef ()

    let opt_ =
        pstring "opt" .>> spaces .>> pchar ':' .>> spaces
        >>. opt (
            between
                (spaces .>> pchar '[' .>> spaces)
                (spaces .>> pchar ']' .>> spaces)
                (sepBy (choice [ bool_; str_; ref_; arr_ ]) (pchar ',' .>> spaces))
        )
        |>> box

    let program =
        between
            (spaces .>> pchar '[' .>> spaces)
            (pchar ']' .>> spaces)
            (sepBy (choice [ bool_; str_; ref_; arr_; opt_ ]) (spaces .>> pchar ',' .>> spaces))
        .>> eof

    do
        arrRef.Value <-
            pstring "arr" .>> spaces .>> pchar ':'
            >>. between
                    (spaces .>> pchar '[' .>> spaces)
                    (spaces .>> pchar ']' .>> spaces)
                    (sepBy (choice [ bool_; str_; ref_; opt_; arr_ ]) (pchar ',' .>> spaces))
            .>> spaces
            |>> box

    member val private AST = [||] with get, set
    member val private Data = [||] with get, set

    static member private parseData(data: string) =
        let ident = attempt (regex "[\w_][\w\d_]*(::[\w_][\w\d_]*)*")

        let bool_ =
            pstring "bool" .>> spaces .>> pchar ':' .>> spaces
            >>. choice [ attempt (stringReturn "true" true)
                         stringReturn "false" false ]
            .>> spaces
            |>> box

        let str_ =
            pstring "str" .>> spaces .>> pchar ':'
            >>. between (spaces .>> pchar '"') (pchar '"' .>> spaces) (manyChars (noneOf "\""))
            .>> spaces
            |>> box

        let ref_ =
            pstring "ref" .>> spaces .>> pchar ':' .>> spaces
            >>. pint32
            .>> spaces
            |>> box

        let arr_, arrRef = createParserForwardedToRef ()

        let opt_ =
            pstring "opt" .>> spaces .>> pchar ':' .>> spaces
            >>. opt (
                between
                    (spaces .>> pchar '[' .>> spaces)
                    (spaces .>> pchar ']' .>> spaces)
                    (sepBy (choice [ bool_; str_; ref_; arr_ ]) (pchar ',' .>> spaces))
            )
            |>> box

        let program =
            between
                (spaces .>> pchar '[' .>> spaces)
                (pchar ']' .>> spaces)
                (sepBy (choice [ bool_; str_; ref_; arr_; opt_ ]) (spaces .>> pchar ',' .>> spaces))
            .>> eof

        arrRef.Value <-
            pstring "arr" .>> spaces .>> pchar ':'
            >>. between
                    (spaces .>> pchar '[' .>> spaces)
                    (spaces .>> pchar ']' .>> spaces)
                    (sepBy (choice [ bool_; str_; ref_; opt_; arr_ ]) (pchar ',' .>> spaces))
            .>> spaces
            |>> box

        match run program data with
        | Success (res, _, _) -> res
        | Failure (msg, _, _) ->
            failwith
            <| sprintf "FlatAST.Data ->! Failured Parse.\n%s" msg

    static member private renderValue(value: obj) =
        let escapeString (text: string) =
            text.Replace("\\", "\\\\").Replace("\"", "\\\"")

        match value with
        | :? bool as value -> sprintf "bool: %b" value
        | :? string as value -> sprintf "str: \"%s\"" (escapeString value)
        | :? int as value -> sprintf "ref: %i" value
        | :? (obj list) as items ->
            sprintf "arr: [%s]" (items |> List.map FlatAST.renderValue |> String.concat ", ")
        | :? (obj list option) as items ->
            match items with
            | Some values -> sprintf "opt: [%s]" (values |> List.map FlatAST.renderValue |> String.concat ", ")
            | None -> "opt: []"
        | _ -> failwith "FlatAST.renderValue ->! Unsupported value."

    static member ParseData(data: string) = FlatAST.parseData data

    static member RenderData(values: obj list) =
        sprintf "[%s]" (values |> List.map FlatAST.renderValue |> String.concat ", ")

    override this.ToString() : string =
        sprintf
            "FlatAST:\n%s\n\n"
            (this.AST
             |> Array.mapi (sprintf "[ %i ]: %A\n")
             |> String.concat "\n")

    member this.add(ast) =
        this.AST <- [| ast |] |> Array.append this.AST
        this.AST.Length - 1

    member this.setAST(ast: ASTNode array) =
        this.AST <- ast
        this.Data <- [||]

    member this.initData() =
        this.Data <-
            this.AST
            |> Array.map (fun ast ->
                FlatAST.parseData ast.Data)

    member this.getAST() = this.AST

    member this.getAST i =
        if i < 0 || this.AST.Length <= i then
            Err("FlatAST.Ast ->! Index Out Of Range.")
        else
            Ok(this.AST[i])

    member this.getData() = this.Data

    member this.getData i =
        if i < 0 || this.Data.Length <= i then
            Err("FlatAST.Data ->! Index Out Of Range.")
        else
            Ok(this.Data[i])
// TODO
