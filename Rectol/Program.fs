(* This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
   Copyright (c) 2026 Rectol Language *)

open Psictre
open Rectol.Lexer
open Rectol
open Test
open Sample2

[<EntryPoint>]
let main _ =
    run() |> printfn "%A"
    0
