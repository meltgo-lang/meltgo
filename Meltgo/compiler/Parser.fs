namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module PublicParser =
    let pnumber = parse {
        let! x, _ = pdigits
        return x |> List.toArray |> string |> int
    }
