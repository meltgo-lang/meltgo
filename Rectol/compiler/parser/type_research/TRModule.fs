namespace Psictre.TypeReserach

open System

[<AutoOpen>]
module TRModule =
    type ITRConstrait =
        abstract member Delegate: target: ITRConstrait -> ITRConstrait option

    type ITRConstraitGetter<'a> =
        abstract member Get: unit -> 'a

    type ITRImpl =
        abstract member Constraits: ITRConstrait list
        abstract member GenFunc: target: ITRConstrait -> (ITRConstrait list -> ITRConstrait option)
