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

    type Node =
        | Package of string
        | Import of string
        | Defvar of string * bool * Expr

    let block, blockRef = refParser<Expr>()

    let spaces =
        parse {
            let! x, _ = many (pchar ' ') |> toStr string
            return x
        }

    let spaces1 =
        parse {
            let! x, _ = many1 (pchar ' ') |> toStr string
            return x
        }

    let pnumber =
        parse {
            let! x, _ = pdigits
            return x |> int
        }

    let pident =
        parse {
            let! x, _ = pletter |> charToStr
            let! y, _ = many (pletter <|> pdigit) |> toStr string
            return x + y
        }

    let pidents =
        parse {
            let! fst, _ = pident
            let! lst, _ = many (parse {
                let! _ = spaces
                let! _ = pstring "::"
                let! _ = spaces
                let! ident, _ = pident
                return ident
            })
            let rec loop =
                function
                | x::xs -> x + loop xs
                | [] -> ""
            return fst + loop (lst |> List.map ((+) "::"))
        }
