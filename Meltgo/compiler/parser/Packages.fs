namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module Packages =
    let package =
        parse {
            let! _ = pstring "package"
            let! _ = spaces1
            let! ident, _ = pidents
            return Package ident
        }

    let import =
        parse {
            let! _ =pstring "import"
            let! _ = spaces1
            let! ident, _ = pidents
            return Import ident
        }
