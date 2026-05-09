namespace Psictre

type TypeConstrait<'a> = 'a -> Result<bool, string>
type Parser<'a> = string -> Result<'a * string, string>

[<AutoOpen>]
module PublicParserFunc =
    let run p input = p input

    let fail = fun _ -> Error ""

module private ParserFunc =
    let result x = fun input -> Ok (x, input)

    let bind f p =
        fun input ->
            match run p input with
            | Ok (v, rest) -> run (f (v, rest)) rest
            | Error e -> Error e

[<AutoOpen>]
module ParserType =
    open ParserFunc

    type TypeDecide() =
        
        member __.MergeSource(x, y) = x, y
        member __.Return(x) = ()

    type ParserBuilder() =
        member __.Bind(p, f) = bind f p
        member __.Return(x) = result x
        member __.ReturnFrom(p) = p
        member __.Zero() = fail

[<AutoOpen>]
module ComputationExpressionForParser =
    open ParserFunc
    open System

    let parse = ParserBuilder()
    let typedec = TypeDecide()

    let attempt (p: Parser<'a>) =
        fun input ->
            match run p input with
            | Ok(res, input) ->
                parse {
                    return res
                }
            | Error e ->
                parse {
                    return! fail
                }

#nowarn 64
    let (<|>) (p1: Parser<'a>) (p2: Parser<'b>): Parser<'c> =
        fun input ->
            match run p1 input with
            | Ok _ as res -> res
            | Error _ -> run p2 input
#warnon 64

    let skip1: Parser<char> =
        fun input ->
            if input = "" then Error ""
            else Ok (input[0], input[1..])

    let skip i: Parser<string> =
        fun input ->
            if input = "" then Error ""
            else Ok (input[0..i-1], input[i..])

    let satisfy1 pred: Parser<char> =
        parse {
            let! c, _ = skip1
            if pred c then return c
            else return! fail
        }

    let satisfy pred i: Parser<string> =
        parse {
            let! c, _ = skip i
            if pred c then return c
            else return! fail
        }

    let pchar c = satisfy1 ((=) c)

    let pstring (s: string) = satisfy ((=) s) s.Length

    let rec many (p: Parser<'a>): Parser<'a list> =
        (parse {
            let! x, _ = p
            let! xs, _ = many p
            return x :: xs
        }) <|> result []

    let many1 (p: Parser<'a>): Parser<'a list> =
        parse {
            let! x, _ = p
            let! xs, _ = many p
            return x :: xs
        }

    let pdigit = satisfy1 Char.IsDigit

    let pint32: Parser<int> =
        parse {
            let! digits, _ = many1 pdigit
            return digits |> List.toArray |> String |> int
        }

    let eof: Parser<unit> =
        fun input ->
            if input = "" then Ok ((), "")
            else Error ""
