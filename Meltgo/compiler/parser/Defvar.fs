namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module DefvarParser =
    let defvar =
        parse {
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
            let! _ = spaces
            let! expr, _ = addsub <|> block
            return vname, isMut, expr
        }
    