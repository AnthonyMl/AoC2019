module Lib
    ( asteroidDetection
    ) where

import Data.Foldable (find)
import Data.List (elemIndices, nub, maximumBy)
import Data.Maybe (isJust)


spaceGrid = [
    ".#..#..##.#...###.#............#.",
    ".....#..........##..#..#####.#..#",
    "#....#...#..#.......#...........#",
    ".#....#....#....#.#...#.#.#.#....",
    "..#..#.....#.......###.#.#.##....",
    "...#.##.###..#....#........#..#.#",
    "..#.##..#.#.#...##..........#...#",
    "..#..#.......................#..#",
    "...#..#.#...##.#...#.#..#.#......",
    "......#......#.....#.............",
    ".###..#.#..#...#..#.#.......##..#",
    ".#...#.................###......#",
    "#.#.......#..####.#..##.###.....#",
    ".#.#..#.#...##.#.#..#..##.#.#.#..",
    "##...#....#...#....##....#.#....#",
    "......#..#......#.#.....##..#.#..",
    "##.###.....#.#.###.#..#..#..###..",
    "#...........#.#..#..#..#....#....",
    "..........#.#.#..#.###...#.....#.",
    "...#.###........##..#..##........",
    ".###.....#.#.###...##.........#..",
    "#.#...##.....#.#.........#..#.###",
    "..##..##........#........#......#",
    "..####......#...#..........#.#...",
    "......##...##.#........#...##.##.",
    ".#..###...#.......#........#....#",
    "...##...#..#...#..#..#.#.#...#...",
    "....#......#.#............##.....",
    "#......####...#.....#...#......#.",
    "...#............#...#..#.#.#..#.#",
    ".#...#....###.####....#.#........",
    "#.#...##...#.##...#....#.#..##.#.",
    ".#....#.###..#..##.#.##...#.#..##"
    ]

data Point = Point Int Int
    deriving (Eq, Show)

data Space = Empty | Asteroid
    deriving (Eq, Show)

fromChar :: Char -> Space
fromChar c
    | c == '#' = Asteroid
    | otherwise = Empty

grid :: [[Space]]
grid = map (map fromChar) spaceGrid

line :: Point -> Point -> [Point]
line (Point ax ay) (Point bx by)
    | ax == bx && ay == by = []
    | ax == bx = if ay < by then [Point ax y | y <- [ay+1..by]]
                            else [Point ax y | y <- [ay-1, (pred (ay-1))..by]]
    | ay == by = if ax < bx then [Point x ay | x <- [ax+1..bx]]
                            else [Point x ay | x <- [ax-1, (pred (ax-1))..bx]]
    | otherwise = let
        rise = by - ay
        run  = abs (bx - ax)
        cx = [1..run]
        xys = [Point x (div (x * rise) run) | x <- cx, mod (x * rise) run == 0]

        xFactor = if bx < ax then (-1) else 1
        xys2 = map (\(Point x y) -> Point (ax + x*xFactor) (ay + y)) xys
    in
        filter (\(Point x y)->(y < length grid) && (x >= 0) && (x < length (head grid)) && (y >= 0)) xys2

-- assumes both are asteroids
visibility :: Point -> Point -> Maybe Point
visibility a b = find (\(Point x y) -> Asteroid == grid !! y !! x) (line a b)

asteroidDetection :: IO ()
asteroidDetection =
    let
        indexedGrid = zip [0..] (map (elemIndices Asteroid) grid)
        asteroids = indexedGrid >>= (\(y, xs) -> map (\x -> Point x y) xs)

        allPairs = map (\a->map (visibility a) asteroids) asteroids

        lengths = map (length . nub . filter isJust) allPairs

        m = maximumBy (\(al,_) (bl,_) -> compare al bl) (zip lengths asteroids)
    in
        print m
