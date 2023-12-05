package aoc

import scala.annotation.targetName

extension [A](a: A) {
  @targetName("pipe")
  inline def |>[B](f: A => B): B = f(a)
}
