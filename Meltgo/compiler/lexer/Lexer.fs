namespace Rectol.Lexer

type Iterator(str: string) =
    let f =
        let mutable count = 0
        fun () ->
            count <- count + 1
            if count <= str.Length
            then Ok str[count - 1]
            else Error <| sprintf "index out of range, '%s'" str

    new() = Iterator("")
    member __.get() = f()

type Lexer(_a: obj, str: string) =
    let parseStr = Iterator(str)
    let memo = Iterator()

    member __.get() =
        parseStr
