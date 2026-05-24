(* This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
   Copyright (c) 2026 Meltgo Language *)

open Psictre
open Psictre.DirectedLambdaTypeInference
open Meltgo.Lexer
open Meltgo

[<EntryPoint>]
let main _ =
    let pp = PseudoPointer()
    let b = BindVar(pp, fun x ->
        x.Type <- Some "Obj"
        x)
    match b.Execute b with
    | :? BindVar<obj> as b ->
        printfn "%A" b.Type
        printfn "%A" (pp.GetMap())
    | _ -> ()
    0
