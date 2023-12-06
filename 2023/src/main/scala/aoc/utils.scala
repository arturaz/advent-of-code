package aoc

import scala.annotation.targetName

extension [A](a: A) {
  @targetName("pipe")
  inline def |>[B](f: A => B): B = f(a)
}

def factorial(n: Long): BigInt = {
  var result = BigInt(1)
  for (i <- 2L to n) {
    result *= i
  }
  result
}