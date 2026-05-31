namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module ParserBase =
    type Expr =
        | Nan
        | Num of int
        | Add of Expr * Expr
        | Sub of Expr * Expr
        | Mul of Expr * Expr
        | Div of Expr * Expr

    let block, blockRef = refParser<Expr>()

    let spaces = parse {
        let! x, _ = many (pchar ' ') |> toStr string
        return x
    }

    let spaces1 = parse {
        let! x, _ = many1 (pchar ' ') |> toStr string
        return x
    }

    let pnumber = parse {
        let! x, _ = pdigits
        return x |> int
    }

    let pident = parse {
        let! x, _ = pletter |> charToStr
        let! y, _ = many (pletter <|> pdigit) |> toStr string
        return x + y
    }
