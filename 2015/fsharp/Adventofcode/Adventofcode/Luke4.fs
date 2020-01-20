module Luke4

//http://adventofcode.com/day/4

let input = "bgvyzdsv"

(* Part 1 *)

//http://www.fssnip.net/lK
let MD5Hash (input : string) =
   use md5 = System.Security.Cryptography.MD5.Create()
   input
   |> System.Text.Encoding.ASCII.GetBytes
   |> md5.ComputeHash
   |> Seq.map (fun c -> c.ToString("X2"))
   |> Seq.reduce (+)

(* Mutable, imperativ version *)
let calcKeyPart prefix = 
    let mutable found = false
    let mutable i = 1
    while not found do
        let md5 = MD5Hash(input + i.ToString())
        if md5.StartsWith(prefix) then found <- true
        else i <- i+1
    i

(* Functional tail-recursive*)
let calcKeypart' prefix =
    let rec inner index =
        let md5 = MD5Hash(input + index.ToString())
        if md5.StartsWith(prefix) then index
        else inner (index+1)
    inner 1

let res = calcKeyPart "00000"
let resRec = calcKeypart' "00000"

(*Part 2*)

let res' = calcKeyPart "000000"
let resRec' = calcKeypart' "000000"
