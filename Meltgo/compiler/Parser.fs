namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module PublicParser =
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

    let defvar = parse {
        let! _ = pstring "let"
        let mutable isMut = false
        let! _ = spaces1
        let! _ = opt (parse {
            let! _ = pstring "mut"
            isMut <- true
            let! _ = spaces1
            return 0
        })
        let! vname, _ = pident
        let! _ = spaces
        let! _ = pchar '='
        return vname, isMut
    }
