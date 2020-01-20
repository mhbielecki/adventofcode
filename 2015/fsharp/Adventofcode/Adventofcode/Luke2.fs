module Luke2

open System
open System.IO

//http://adventofcode.com/day/2

let input = File.ReadAllLines(@"C:\Users\martinb\Dropbox\Programming\adventofcode\fsharp\Adventofcode\Adventofcode\2_input.txt")

(* Part 1 - Gift wrapping paper *)
type Gift = {
    length: int;
    width: int;
    height: int;
}

//2*l*w + 2*w*h + 2*h*l
let findPaperAmount (gift:Gift) =
    let l,w,h = gift.length, gift.width, gift.height
    let surface = 2*(l*w + w*h + h*l)
    let extra = List.min [l*w; w*h; h*l]
    surface + extra

// Approach 1: Make gifts using multiple mappings
let gifts = 
    input
    |> Seq.map(fun s -> s.Split('x'))
    |> Seq.map(fun l -> l |> Seq.map int |> Seq.toArray)
    |> Seq.map(fun l -> { length = l.[0]; width = l.[1]; height = l.[2]})

let totalPaperAmount = 
    gifts
    |> Seq.map(fun g -> findPaperAmount g)
    |> Seq.sum
   
//Approach 2: Function composition
let createGifts input =
    let splitDims (s:string) = s.Split('x')
    let dimensions l = l |> Seq.map int |> Seq.toArray
    let makeGifts (values:int[]) = { length = values.[0]; width = values.[1]; height = values.[2]}
    Seq.map (fun gift -> (splitDims >> dimensions >> makeGifts) gift) input

let totalPaperAmount' = 
    input
    |> createGifts
    |> Seq.map(fun g -> findPaperAmount g)
    |> Seq.sum

// Tests
let g1 = {length = 2; width = 3; height = 4; }
let test1 = findPaperAmount g1

let g2 = {length = 1; width = 1; height = 10; }
let test2 = findPaperAmount g2

test1 = 58
test2 = 43

(* Part 2 - Gift ribbon *)

let findRibbonLength gift =
    let l,w,h = gift.length, gift.width, gift.height
    let sides = 
        [l;w;h] 
        |> List.sort 
        |> List.take 2
        |> List.sum
    sides*2 + l*w*h

let totalRibbonLength = 
    gifts
    |> Seq.map(fun g -> findRibbonLength g)
    |> Seq.sum

// Tests
let g3 = {length = 2; width = 3; height = 4; }
let test3 = findRibbonLength g3

let g4 = {length = 1; width = 1; height = 10; }
let test4 = findRibbonLength g4

test1 = 34
test2 = 14
