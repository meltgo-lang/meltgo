namespace Psictre.TypeReserach

open System

[<AutoOpen>]
module TRModule =
    type ITRTypeConstrait =
        abstract member Delegate: target: ITRTypeConstrait -> ITRTypeConstrait option

    type ITRTypeConstraitGetter<'a> =
        abstract member Get: unit -> 'a

    type ITRImpl =
        abstract member Constraits: ITRTypeConstrait
        abstract member Mapping: input: ITRTypeConstrait -> bool
        abstract member GenerateFunc: target: ITRTypeConstrait -> (ITRTypeConstrait list -> ITRTypeConstrait option)

