module Luke3
open System
open System.IO

//http://adventofcode.com/day/3

let input = File.ReadAllText(@"C:\Users\martinb\Dropbox\Programming\adventofcode\fsharp\Adventofcode\Adventofcode\3_input.txt").ToCharArray() |> Array.toList

(* Part 1 - Number of houses receiving at least one present *)

let rec move (houses:Set<int*int>) input (x, y) =
    match input with
    | [] -> houses
    | n::ns -> 
        let h = match n with
                | '^' -> (x, y+1)
                | 'v' -> (x, y-1)
                | '<' -> (x-1, y)
                | '>' -> (x+1, y)
                | _ -> (x,y)
        move (houses.Add h) ns h
        
let houses = move (Set.empty<int*int>.Add (0,0)) input (0,0) |> Set.count

(* Part 2 *)

//x - the elem, ([], []) - the accumulator. For each elem prepend it to the first list in the tuple
//and return a new tuple where the lists have been swapped.
//http://stackoverflow.com/a/7945580
let splitList list = List.foldBack (fun x (l,r) -> x::r, l) list ([],[])

let santa, robo = splitList input
let santaHouses = move (Set.empty<int*int>.Add (0,0)) santa (0,0)
let roboHouses = move (Set.empty<int*int>.Add (0,0)) robo (0,0)
let res = Set.union santaHouses roboHouses |> Set.count

res = 2341