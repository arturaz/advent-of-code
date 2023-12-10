package aoc

import org.locationtech.jts.geom.{Coordinate, GeometryFactory, Polygon}

import scala.annotation.{tailrec, targetName}
import scala.collection.Factory

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

extension [C[X] <: collection.immutable.Seq[X], A](lists: C[LazyList[A]]) {
  def zipTogether(implicit factory: Factory[A, C[A]]): LazyList[C[A]] = {
    val iters = lists.map(_.iterator)
    LazyList.continually(iters.map(_.nextOption).collect { case Some(x) => x }.to(factory)).takeWhile(_.nonEmpty)
  }
}

@tailrec
def greatestCommonDenominator(a: Long, b: Long): Long = {
  if (b == 0) a else greatestCommonDenominator(b, a % b)
}

def lowestCommonMultiplier(a: Long, b: Long): Long = {
  (a / greatestCommonDenominator(a, b)) * b
}

extension (factory: GeometryFactory) {
  def polygonOf(coords: Iterator[Coords]): Polygon = {
    val jtsCoords = coords.map(_.asJTS).toArray
    factory.createPolygon(jtsCoords)
  }
}