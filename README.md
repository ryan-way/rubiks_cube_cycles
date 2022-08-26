# Rubiks Cube Cycles

## Background

Rubiks cube have an interesting property:
From a solved cube, any move or combination of moves that are repeated 
will eventually return the cube to a solved state.

This small application is used to simulate this scenario across 
very combination of moves. 

## Motivation

Use rust for something. Full time from conception to completion was about a week
outside of work.

## Implementation

For depth of 1..n
  perform dfs on the list of moves
  when depth is reached, perform construct list of moves on solved cube
    until the cube returns to solved state
  print depth, number of iterations, and move list

## Results

Because of the explosive nature of a rubiks cube's search space, my machine
left running over night could only reach a depth of 6.

Below are the largest 10 cycles of depths 1 to 6. Largest cycle being defined
as the number of iterations required for a move lost to return the cube
to a solved state.

| Depth | Iterations | Move List                  |
|------ | ---------- | -------------------------- |
|     1 |          4 |                    U_Prime |   
|     1 |          4 |                          U |         
|     1 |          4 |                    S_Prime |   
|     1 |          4 |                          S |         
|     1 |          4 |                    R_Prime |   
|     1 |          4 |                          R |         
|     1 |          4 |                    M_Prime |   
|     1 |          4 |                          M |         
|     1 |          4 |                    L_Prime |   
|     1 |          4 |                          L |         
|     2 |         35 |                         UR |        
|     2 |         35 |             U_PrimeR_Prime |
|     2 |         35 |                   U_PrimeL |  
|     2 |         35 |                   U_PrimeF |  
|     2 |         35 |             U_PrimeB_Prime |
|     2 |         35 |                   UL_Prime |  
|     2 |         35 |                   UF_Prime |  
|     2 |         35 |                         UB |        
|     2 |         35 |                         RU |        
|     2 |         35 |             R_PrimeU_Prime |
|     3 |        140 |                  US_PrimeB | 
|     3 |        140 |                  USF_Prime | 
|     3 |        140 |                  URM_Prime | 
|     3 |        140 |                  URE_Prime | 
|     3 |        140 |            U_PrimeS_PrimeF |
|     3 |        140 |            U_PrimeSB_Prime |
|     3 |        140 |            U_PrimeR_PrimeM |
|     3 |        140 |            U_PrimeR_PrimeE |
|     3 |        140 |            U_PrimeMR_Prime |
|     3 |        140 |            U_PrimeM_PrimeL |
|     4 |        420 |     US_PrimeF_PrimeB_Prime |
|     4 |        420 |           US_PrimeF_PrimeB |
|     4 |        420 |           US_PrimeFL_Prime |
|     4 |        420 |           US_PrimeFB_Prime |
|     4 |        420 |                 US_PrimeFB |
|     4 |        420 |     US_PrimeB_PrimeF_Prime |
|     4 |        420 |           US_PrimeB_PrimeF |
|     4 |        420 |           US_PrimeBF_Prime |
|     4 |        420 |                 US_PrimeBF |
|     4 |        420 |           USF_PrimeB_Prime |
|     5 |        420 |    UUS_PrimeR_PrimeD_Prime |
|     5 |        420 |                UUS_PrimeRD |
|     5 |        420 |          UUS_PrimeF_PrimeD |
|     5 |        420 |    UUS_PrimeD_PrimeL_Prime |
|     5 |        420 |    UUS_PrimeD_PrimeB_Prime |
|     5 |        420 |                UUS_PrimeDL |
|     5 |        420 |          UUS_PrimeDF_Prime |
|     5 |        420 |    UUS_PrimeB_PrimeD_Prime |
|     5 |        420 |                UUSL_PrimeD |
|     5 |        420 |                UUSLD_Prime |
|     6 |        420 |   UUUS_PrimeF_PrimeB_Prime |
|     6 |        420 |         UUUS_PrimeF_PrimeB |
|     6 |        420 |         UUUS_PrimeFB_Prime |
|     6 |        420 |               UUUS_PrimeFB |
|     6 |        420 |   UUUS_PrimeB_PrimeF_Prime |
|     6 |        420 |         UUUS_PrimeB_PrimeF |
|     6 |        420 |               UUUS_PrimeBL |
|     6 |        420 |         UUUS_PrimeBF_Prime |
|     6 |        420 |               UUUS_PrimeBF |
|     6 |        420 |         UUUSF_PrimeR_Prime |


## Refections

There's a couple improvements that can be done.

Firstly, if I did any research into the math behind this property, it could
probably be taken advantage of the reduce the search space

Second, all the operations on the 3d array are implemented from scratch. While
the tranposes and reverses are fast, the rotations on the cube invole some iterator
magic and follow a read, update, then write pattern. There are probably some good libraries
out there for this kind of manipulation
