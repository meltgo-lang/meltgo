namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module PublicParser =
    let pnumber = parse {
        let! x, _ = pdigits
        return x |> int
    }

    let pident = parse {
        let! x, _ = pletter
        let! y, _ = many (pletter <|> pdigit)
        return x::y |> List.map _.ToString() |> List.reduce (+)
    }

    let defvar = parse {
        let! _ = pstring "let"
        let! _ = many1 (pchar ' ')
        let! vname, _ = pident
        return vname
    }
