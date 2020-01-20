module Luke1

open System
open System.IO

// http://adventofcode.com/day/1

let input = File.ReadAllText(@"C:\Users\martinb\Dropbox\Programming\adventofcode\fsharp\Adventofcode\Adventofcode\1_input.txt").ToCharArray()

(* Part 1 *) 

// First solution: partition into two lists and subtracting the lengths
let up, down = Array.partition(fun x -> x = '(') input
let floor = up.Length - down.Length

// Second solution: Mapping parenthesis to numbers and summing
let flor = 
    input
    |> Seq.map(fun x -> if x = '(' then 1 else -1)
    |> Seq.sum

//Third solution: use sumBy
let flr = Seq.sumBy (fun x -> if x = '(' then 1 else -1) input

(* Part 2 *)
let findFirstNegativeFloor commands = 
    let rec inner commands floor charposition = 
        if floor = -1 then charposition
        else 
            match commands with
            | '('::xs -> inner xs (floor+1) charposition+1
            | ')'::xs -> inner xs (floor-1) charposition+1
    inner commands 0 0

let pos = findFirstNegativeFloor (input |> Array.toList)


let test1 = ['(';')';'(';')';')']
let test2 = [')']
findFirstNegativeFloor test1 = 5
findFirstNegativeFloor test2 = 1