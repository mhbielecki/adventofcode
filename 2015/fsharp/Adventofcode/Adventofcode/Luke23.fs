module Luke23
open System
open System.IO

//http://adventofcode.com/day/23

let input = File.ReadAllLines(@"C:\Users\martinb\Dropbox\Programming\adventofcode\fsharp\Adventofcode\Adventofcode\23_input.txt")

(* Part 1 *)

type Register = 
    | A 
    | B

type Instruction = 
    | Hlf of Register
    | Tpl of Register
    | Inc of Register
    | Jmp of int
    | Jie of Register * int
    | Jio of Register * int
    | Unknown

let makeInstruction (s:string) =
    let getReg s = if s = "a" then A else B
    let parts = s.Replace(",", "").Split(' ')
    let inst = parts.[0]
    match inst with
        | "hlf" -> Hlf(getReg(parts.[1]))
        | "tpl" -> Tpl(getReg(parts.[1]))
        | "inc" -> Inc(getReg(parts.[1]))
        | "jmp" -> Jmp(parts.[1] |> int)
        | "jie" -> Jie(getReg(parts.[1]), parts.[2] |> int)
        | "jio" -> Jio(getReg(parts.[1]), parts.[2] |> int)
        | _ -> Unknown

let rec compute (instructions:Instruction[]) index regA regB =
    if index < 0 || index >= instructions.Length then regB else
    let currentInstruction = instructions.[index]
    match currentInstruction with
    | Hlf(reg) -> 
        match reg with 
        | A -> compute instructions (index+1) (regA/2) regB
        | B -> compute instructions (index+1) regA (regB/2)
    | Tpl(reg) -> 
        match reg with 
        | A -> compute instructions (index+1) (regA*3) regB
        | B -> compute instructions (index+1) regA (regB*3)
    | Inc(reg) -> 
        match reg with 
        | A -> compute instructions (index+1) (regA+1) regB
        | B -> compute instructions (index+1) regA (regB+1)
    | Jmp(offset) -> compute instructions (index+offset) regA regB
    | Jie(reg, offset) -> 
        match reg with 
        | A when (regA % 2 = 0) -> compute instructions (index+offset) regA regB
        | B when (regB % 2 = 0) -> compute instructions (index+offset) regA regB
        | _ -> compute instructions (index+1) regA regB
    | Jio(reg, offset) -> 
        match reg with 
        | A when (regA = 1) -> compute instructions (index+offset) regA regB
        | B when (regB = 1) -> compute instructions (index+offset) regA regB
        | _ -> compute instructions (index+1) regA regB
    | Unknown -> regB

let instructions = input |> Seq.map makeInstruction |> Seq.toArray
let res = compute instructions 0 0 0

res = 255
(* Part 2 *)

let res' = compute instructions 0 1 0
res' = 334
