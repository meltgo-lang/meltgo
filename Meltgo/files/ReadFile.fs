namespace Meltgo

open System.IO

type ReadFile(extension) =
    member __.Read(path: string) =
        if path.EndsWith(sprintf ".%s" extension) then
            Error ""
        else
            use sr = new StreamReader(path)
            Ok(sr.ReadToEnd())
