namespace Psictre.MonadicLambdaWalkingTypeInference

[<AutoOpen>]
module PublicMLW =
    type IMLWUnion = interface end

    type PseudoPointer<'T when 'T :> IMLWUnion and 'T: equality>() =
        let mutable map = [||]
        let mutable typs = [||]

        member __.Add (d: 'T) =
            let id = map |> Array.length
            map <- [|
                if map |> Array.isEmpty then 0u, d
                else (map |> Array.map fst |> Array.max |> (+) 1u), d
            |] |> Array.append map
            typs <- [|
                None
            |] |> Array.append typs
            id

        member __.Unification(targetId: int)(valueId: int) =
            let target = map[targetId]
            let value = map[valueId]
            map <-
                map
                |> Array.map (fun x ->
                    if x = target then value
                    else x
                )

        member __.SetType(id: int)(t: string) =
            typs[id] <- Some t

        member __.GetMap() = map

        member __.GetResult() = typs

    type IBindVarFunction =
        abstract GetFunc: unit -> obj

#nowarn 64
    type TypeVar<'a, 'b when 'a :> IMLWUnion and 'a: equality and 'b :> IBindVarFunction>(pp: PseudoPointer<'a>, d: 'a, f: 'b) =
        let id = pp.Add d

        member private __.GetId() = id

        member __.GetFunc<'T>() = f :> IBindVarFunction |> _.GetFunc() |> unbox : 'T

        member __.SetType(t: string) = pp.SetType id t

        member __.Unification<'c, 'd when 'c :> IMLWUnion and 'c: equality and 'd :> IBindVarFunction>(b: TypeVar<'c, 'd>) =
            pp.Unification id (b.GetId())
#warnon 64
