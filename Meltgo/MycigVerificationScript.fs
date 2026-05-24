namespace Mycig.VerificationScript

open FParsec

type Type =
    | TVar of string
    | TCon of string * Type list

type MemoryStatus = string

type Constraint =
    | Equal of Type * Type
    | Transition of string * MemoryStatus * MemoryStatus

type Statement =
    | StateDef of string list
    | RuleDef of string * (MemoryStatus * MemoryStatus)
    | InferDef of string * Type * Constraint list

type Program = Statement list

module VSParser =
    let ws = spaces
    let str_ws s = pstring s .>> ws
    let char_ws c = pchar c .>> ws

    let identifier: Parser<string, unit> =
        many1Satisfy2L isLetter (fun c -> isLetter c || isDigit c || c = '_') "identifier"
        .>> ws

    let pType, pTypeRef = createParserForwardedToRef<Type, unit> ()

    let comment =
        parse {
            do! pstring "//" |>> ignore
            do! skipMany newline
        }

    let pTCon =
        pipe2 identifier (opt (between (char_ws '<') (char_ws '>') (sepBy pType (char_ws ',')))) (fun name args ->
            TCon(name, defaultArg args []))

    let pTVar = char_ws '\'' >>. identifier |>> TVar
    do pTypeRef.Value <- pTVar <|> pTCon

    let pStateDef =
        str_ws "states"
        >>. between (char_ws '{') (char_ws '}') (sepBy identifier (char_ws ',' .>> optional (comment)))
        |>> StateDef

    let pRuleDef =
        parse {
            do! str_ws "rule" |>> ignore
            let! name = identifier

            return!
                between
                    (char_ws '{')
                    (char_ws '}')
                    (parse {
                        let! src = identifier
                        do! str_ws "->" |>> ignore
                        let! tgt = identifier
                        return RuleDef(name, (src, tgt))
                    })
        }

    let pConstraint =
        attempt (pipe2 pType (str_ws "==" >>. pType) (fun a b -> Equal(a, b)))
        <|> (parse {
            let! act = identifier

            let! (s, t) =
                between (char_ws '(') (char_ws ')') (pipe2 identifier (char_ws ',' >>. identifier) (fun s t -> (s, t)))

            return Transition(act, s, t)
        })

    let pInferDef =
        parse {
            do! str_ws "infer" |>> ignore
            let! name = identifier
            do! char_ws ':' |>> ignore
            let! t = pType
            let! constraints = between (char_ws '{') (char_ws '}') (many (pConstraint .>> optional (char_ws ';')))
            return InferDef(name, t, constraints)
        }

    let pProgram =
        ws
        >>. many (
            pStateDef <|> pRuleDef <|> pInferDef
            .>> optional (comment)
        )
        .>> eof

module VSEngine =
    let rec substitute (subst: Map<string, Type>) =
        function
        | TVar n ->
            Map.tryFind n subst
            |> Option.defaultValue (TVar n)
        | TCon (n, args) -> TCon(n, List.map (substitute subst) args)

    let rec unify (t1: Type) (t2: Type) (subst: Map<string, Type>) : Map<string, Type> option =
        let t1, t2 = substitute subst t1, substitute subst t2

        match t1, t2 with
        | _ when t1 = t2 -> Some subst
        | TVar n, t
        | t, TVar n -> Some(Map.add n t subst)
        | TCon (n1, a1), TCon (n2, a2) when n1 = n2 && a1.Length = a2.Length ->
            List.fold2 (fun s x y -> s |> Option.bind (unify x y)) (Some subst) a1 a2
        | _ -> None

    let runAnalysis (prog: Program) =
        let mutable subst = Map.empty

        let rules =
            prog
            |> List.choose (function
                | RuleDef (n, r) -> Some(n, r)
                | _ -> None)
            |> Map.ofList

        prog
        |> List.iter (function
            | StateDef ss -> printfn "[Def] Registered States: %s" (String.concat ", " ss)
            | RuleDef (n, (s, t)) -> printfn "[Def] Rule %s: %s -> %s" n s t
            | InferDef (name, t, constraints) ->
                printfn "[Check] Inferring: %s" name

                constraints
                |> List.iter (function
                    | Equal (a, b) ->
                        match unify a b subst with
                        | Some s ->
                            subst <- s
                            printfn "  (Type) Success: %A" (substitute subst a)
                        | None -> printfn "  (Type) [Error] Mismatch: %A != %A" a b
                    | Transition (act, fromS, toS) ->
                        match Map.tryFind act rules with
                        | Some (rFrom, rTo) when rFrom = fromS && rTo = toS ->
                            printfn "  (Memory) [OK] %s: %s -> %s" act fromS toS
                        | _ -> printfn "  (Memory) [Error] Illegal transition: %s (%s -> %s)" act fromS toS))
