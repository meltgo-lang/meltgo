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
    | DSome
    interface IMLWUnion

type Func =
    | FSingle of (TypeVar<DU, Func> -> TypeVar<DU, Func>)
    | FDouble of (TypeVar<DU, Func> -> TypeVar<DU, Func> -> TypeVar<DU, Func>)
    interface IBindVarFunction with
        member this.GetFunc () =
            match this with
            | FSingle f -> box f
            | FDouble f -> box f

[<EntryPoint>]
let main _ =
    let pp = PseudoPointer()
    let rec b = TypeVar(pp, DSome, FSingle(fun x ->
        x.SetType "Obj"
        x))
    let b2 = TypeVar(pp, DNone, FDouble(fun x y ->
        x.Unification y
        x
    ))

    let res = b.GetFunc<TypeVar<DU, Func> -> TypeVar<DU, Func>>() b
    let res2 = b2.GetFunc<TypeVar<DU, Func> -> TypeVar<DU, Func> -> TypeVar<DU, Func>>() b2 res
    
    printfn "%A" (pp.GetMap())
    printfn "%A" (pp.GetResult())

    run (parse {
        let! res, _ = defvar
        let! _ = eof
        return res
    }) "let a = 1*2+3/4" |> printfn "%A"

    0
