#eval String.append "Hello, " "Lean!"

#eval String.append "It is " (if 1 >  2 then "yes" else "no")

#eval 42 + 19
#eval String.append "A" (String.append "B" "C")
#eval if 3 == 3 then 5 else 7


-- 1.2 Types
#eval ( 1 + 2  : Nat)
#eval ( 1 - 2  : Nat)
#eval ( 1 - 2 : Int)
#check ( 1 - 2 : Int)
#check String.append ["hello" , " "] " world"

-- 1.3 Functions and definitions
def hello := "Hello"
def lean : String := "Lean"
#eval String.append hello (String.append " " lean)

def add1( n : Nat) : Nat := n + 1
#eval add1 7

def maximum ( n : Nat) (k : Nat) : Nat :=
  if n < k then
    k
  else n

#eval maximum (6 - 1 ) (9 + 12)

def joinStringsWith (first: String) (second : String) (third : String) : String :=
  String.append second (String.append first third)

#eval joinStringsWith "," "one" "and another"

def Str: Type := String

#eval 1 % 26
