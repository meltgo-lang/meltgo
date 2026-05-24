namespace Psictre.DirectedLambdaTypeInference

[<AutoOpen>]
module PublicDLTI =
    type PseudoPointer() =
        let mutable map = [||]

        member __.Add() =
            let id = map |> Array.length
            map <- map |> Array.append [|
                if map |> Array.isEmpty then 0u
                else map |> Array.max |> (+) 1u
            |]
            id

        member __.Unification(target: uint, value: uint) =
            map <-
                map
                |> Array.map (fun x ->
                    if x = target then value
                    else x
                )

        member __.GetMap() = map

    type BindVar<'a>(pp: PseudoPointer, f: BindVar<'a> -> 'b) =
        let mutable id = pp.Add()

        member val Type: string option = None with get, set

        member this.Execute = f
