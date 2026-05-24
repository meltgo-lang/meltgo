namespace Mycig.Compiler

open System.Collections.Generic

type SimpleScope() =
    let mutable scope = [||]
    let mutable len = -1

    member __.createScope() =
        scope <- scope |> Array.append [| [||] |]
        len <- len + 1

    member __.deleteScope() =
        if len >= 0 then
            scope <- scope[..len]
            len <- len - 1

    member __.add(s: string) =
        if scope |> Array.concat |> Array.contains s |> not then
            scope[len] <- scope[len] |> Array.append [| s |]

    member __.content(s: string) =
        scope |> Array.concat |> Array.contains s
