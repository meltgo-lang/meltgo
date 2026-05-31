(* This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
   Copyright (c) 2026 Rectol Language *)

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

    type ParserBuilder() =
        member __.Bind(p, f) = bind f p
        member __.Return(x) = result x
        member __.ReturnFrom(p) = p
        member __.Zero() = fail

[<AutoOpen>]
module ComputationExpressionForParser =
    open ParserFunc

    let parse = ParserBuilder()

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

    let toStr (f: 'a -> string) (p: Parser<'a list>) =
        parse {
            let! lst, _ = p
            return if lst |> List.isEmpty then "" else lst |> List.map f |> List.reduce (+)
        }

    let charToStr (p: Parser<char>) =
        parse {
            let! c, _ = p
            return c |> string
        }

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

    let opt (p: Parser<'a>) =
        fun input ->
            match run p input with
            | Ok (res, rest) -> Ok (Some res, rest)
            | Error _ -> Ok (None, input)

    let pdigit = satisfy1 System.Char.IsDigit

    let pdigits =
        parse {
            let! x, _ = many1 pdigit
            return x |> List.map _.ToString() |> List.reduce (+)
        }

    let pletter =
        parse {
            let! c, _ = skip1
            let unicode = c |> int
            if
                (0x0041 <= unicode && unicode <= 0x007a)
                || (0x3040 <= unicode && unicode <= 0x309f)
                || (0x30a0 <= unicode && unicode <= 0x30ff)
                || (0x4e00 <= unicode && unicode <= 0x9fff)
            then return c
            else return! fail
        }

    let pletters =
        parse {
            let! x, _ = many1 pletter
            return x |> List.map _.ToString() |> List.reduce (+)
        }

    let eof: Parser<unit> =
        fun input ->
            if input = "" then Ok ((), "")
            else Error ""
