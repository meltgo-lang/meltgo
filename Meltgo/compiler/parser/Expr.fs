namespace Meltgo.Compiler.Parser

open Psictre

[<AutoOpen>]
module ExprParser =
    let penum =
        parse {
            let! i, _ = pnumber |> proc (fun (i, _) -> Num i)
            return i
        }

    let muldiv =
        parse {
            let! l1, _ = penum
            let! lst, _ = many1 (parse {
                let! _ = spaces
                let! op, _ = pchar '*' <|> pchar '/'
                let! _ = spaces
                let! l2, _ = penum
                return op, l2
            })
            let rec loop n =
                function
                | x::xs ->
                    match fst x with
                    | '*' -> loop (Mul (n, snd x)) xs
                    | '/' -> loop (Div (n, snd x)) xs
                    | _ -> Nan
                | [] -> n
            return loop l1 lst
        }

    let addsub =
        parse {
            let! l1, _ = muldiv <|> penum
            let! lst, _ = many1 (parse {
                let! _ = spaces
                let! op, rest = pchar '+' <|> pchar '-'
                let! _ = spaces
                let! l2, _ = muldiv <|> penum
                return op, l2
            })
            let rec loop n =
                function
                | x::xs ->
                    match fst x with
                    | '+' -> loop (Add (n, snd x)) xs
                    | '-' -> loop (Sub (n, snd x)) xs
                    | _ -> Nan
                | [] -> n
            return loop l1 lst
        }
