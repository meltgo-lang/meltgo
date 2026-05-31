(* This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
   Copyright (c) 2026 Meltgo Language *)

open Psictre
open Psictre.MonadicLambdaWalkingTypeInference
open Meltgo
open Meltgo.Compiler.Parser

type DU =
    | DNone
    interface IMLWUnion

type Func =
    | FSingle of (BindVar<DU, Func> -> BindVar<DU, Func>)
    | FDouble of (BindVar<DU, Func> -> BindVar<DU, Func> -> BindVar<DU, Func>)
    interface IBindVarFunction with
        member this.GetFunc () =
            match this with
            | FSingle f -> box f
            | FDouble f -> box f

[<EntryPoint>]
let main _ =
    let pp = PseudoPointer()
    let rec b = BindVar(pp, DNone, FSingle(fun x ->
        x.SetType "Obj"
        x))
    let b2 = BindVar(pp, DNone, FDouble(fun x y ->
        x.Unification y
        x
    ))

    let res = b.GetFunc<BindVar<DU, Func> -> BindVar<DU, Func>>() b
    let res2 = b2.GetFunc<BindVar<DU, Func> -> BindVar<DU, Func> -> BindVar<DU, Func>>() b2 res
    
    printfn "%A" (pp.GetMap())
    printfn "%A" (pp.GetResult())

    run defvar "let aあ" |> printfn "%A"

    0
