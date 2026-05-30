(* This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
   Copyright (c) 2026 Meltgo Language *)

open Psictre
open Psictre.MonadicLambdaHindleyMilnerTypeInference
open Meltgo

type DU =
    | DNone
    interface IMLHMUnion<DU> with
        member __.Default (): DU = 
            DNone

type Func =
    | DSingle of (BindVar<DU, Func> -> BindVar<DU, Func>)
    | DDouble of (BindVar<DU, Func> -> BindVar<DU, Func> -> BindVar<DU, Func>)
    interface IBindVarFunction with
        member this.Mapping (): obj = 
            match this with
            | DSingle f -> box f
            | DDouble f -> box f

[<EntryPoint>]
let main _ =
    let pp = PseudoPointer()
    let rec b = BindVar(pp, DNone, DSingle(fun x ->
        x.SetType "Obj"
        let b2 = BindVar(pp, DNone, DDouble(fun x y ->
            x.Unification y
            x))
        (b2.Execute.Mapping() |> unbox : (BindVar<DU, Func> -> BindVar<DU, Func> -> BindVar<DU, Func>)) b2 b))
    
    let res = (b.Execute.Mapping() |> unbox : (BindVar<DU, Func> -> BindVar<DU, Func>)) b
    printfn "%A" (pp.GetMap())
    printfn "%A" (pp.GetResult())
    0
