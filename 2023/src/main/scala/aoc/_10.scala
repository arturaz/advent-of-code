package aoc

import cats.Show
import org.locationtech.jts.geom.GeometryFactory
import scalax.collection.GraphTraversal.Visitor
import scalax.collection.edges.*
import scalax.collection.immutable.Graph
import scala.collection.parallel.CollectionConverters.*

object Solution10 {
  enum Pipe {
    case Vertical, Horizontal, TopRight, TopLeft, BottomRight, BottomLeft

    lazy val connections: Vector[Direction] = this match {
      case Pipe.Vertical => Vector(Direction.Up, Direction.Down)
      case Pipe.Horizontal => Vector(Direction.Left, Direction.Right)
      case Pipe.TopRight => Vector(Direction.Up, Direction.Right)
      case Pipe.TopLeft => Vector(Direction.Up, Direction.Left)
      case Pipe.BottomRight => Vector(Direction.Down, Direction.Right)
      case Pipe.BottomLeft => Vector(Direction.Down, Direction.Left)
    }

    def connectionsTo[V](position: Coords, map: Map[Coords, V])(using V <:< Pipe): Iterator[(Coords, V)] = {
      connections.iterator.flatMap { direction =>
        val positionInThatDirection = position + direction
        map.get(positionInThatDirection).flatMap { pipeInThatDirection =>
          if (connectsTo(direction, pipeInThatDirection)) Some(positionInThatDirection -> pipeInThatDirection)
          else None
        }
      }
    }

    def toCharSingle: Char = this match {
      case Pipe.Vertical => '│'
      case Pipe.Horizontal => '─'
      case Pipe.TopRight => '└'
      case Pipe.TopLeft => '┘'
      case Pipe.BottomRight => '┌'
      case Pipe.BottomLeft => '┐'
    }

    def toCharDouble: Char = this match {
      case Pipe.Vertical => '║'
      case Pipe.Horizontal => '═'
      case Pipe.TopRight => '╚'
      case Pipe.TopLeft => '╝'
      case Pipe.BottomRight => '╔'
      case Pipe.BottomLeft => '╗'
    }

    def toChar: Char = toCharDouble

    def connectsTo(direction: Direction, pipeInThatDirection: Pipe): Boolean = {
      (this, direction, pipeInThatDirection) match {
        case (Pipe.Vertical, Direction.Up, Pipe.Vertical | Pipe.BottomLeft | Pipe.BottomRight) => true
        case (Pipe.Vertical, Direction.Down, Pipe.Vertical | Pipe.TopLeft | Pipe.TopRight) => true
        case (Pipe.Horizontal, Direction.Left, Pipe.Horizontal | Pipe.TopRight | Pipe.BottomRight) => true
        case (Pipe.Horizontal, Direction.Right, Pipe.Horizontal | Pipe.TopLeft | Pipe.BottomLeft) => true
        case (Pipe.TopRight, Direction.Up, Pipe.Vertical | Pipe.BottomRight | Pipe.BottomLeft) => true
        case (Pipe.TopRight, Direction.Right, Pipe.Horizontal | Pipe.TopLeft | Pipe.BottomLeft) => true
        case (Pipe.TopLeft, Direction.Up, Pipe.Vertical | Pipe.BottomLeft | Pipe.BottomRight) => true
        case (Pipe.TopLeft, Direction.Left, Pipe.Horizontal | Pipe.TopRight | Pipe.BottomRight) => true
        case (Pipe.BottomRight, Direction.Down, Pipe.Vertical | Pipe.TopRight | Pipe.TopLeft) => true
        case (Pipe.BottomRight, Direction.Right, Pipe.Horizontal | Pipe.TopLeft | Pipe.BottomLeft) => true
        case (Pipe.BottomLeft, Direction.Down, Pipe.Vertical | Pipe.TopLeft | Pipe.TopRight) => true
        case (Pipe.BottomLeft, Direction.Left, Pipe.Horizontal | Pipe.TopRight | Pipe.BottomRight) => true
        case _ => false
      }
    }

    def connectsTo(direction: Direction, spotInThatDirection: Spot): Boolean = {
      spotInThatDirection match {
        case Spot.OfPipe(pipe) => connectsTo(direction, pipe)
        case Spot.Ground(_) => false
        case Spot.StartingPosition => false
      }
    }
  }
  object Pipe {
    def parse(c: Char): Option[Pipe] = c match {
      case '|' => Some(Vertical)
      case '-' => Some(Horizontal)
      case 'L' => Some(TopRight)
      case 'J' => Some(TopLeft)
      case '7' => Some(BottomLeft)
      case 'F' => Some(BottomRight)
      case _ => None
    }

    given Show[Pipe] = _.toChar.toString
  }

  enum Spot {
    case OfPipe(pipe: Pipe)
    case Ground(isInner: Boolean)
    case StartingPosition

    def isInnerGround: Boolean = this match {
      case Ground(isInner) => isInner
      case _ => false
    }

    def toChar: Char = this match {
      case Spot.OfPipe(pipe) => pipe.toChar
      case Spot.Ground(isInner) => if (isInner) '.' else ' '
      case Spot.StartingPosition => '╳'
    }
  }
  object Spot {
    def parse(c: Char): Option[Spot] = c match {
      case '.' => Some(Ground(isInner = false))
      case 'S' => Some(StartingPosition)
      case _ => Pipe.parse(c).map(OfPipe.apply)
    }

    given Show[Spot] = _.toChar.toString
  }

  def parse(line: String): Vector[Spot] = {
    line.iterator.map(c => Spot.parse(c).getOrElse(throw new Exception(s"Unknown '$c'"))).toVector
  }

  def parse(lines: Vector[String]): Vector[Vector[Spot]] =
    lines.map(parse)

  def toMap(spots: Vector[Vector[Spot]]): SpotsMap = {
    val m = spots.iterator.zipWithIndex.flatMap { case (row, y) =>
      row.iterator.zipWithIndex.map { case (spot, x) =>
        Coords(x, y) -> spot
      }
    }
    m.toMap
  }

  type SpotsMap = Map[Coords, Spot]
  type PipesMap = Map[Coords, Pipe]

  def startingPositionCoords(map: SpotsMap): Option[Coords] =
    map.find(_._2 == Spot.StartingPosition).map(_._1)

  def determinePipeTypeOfStartingPosition(map: SpotsMap, startingPosition: Coords): Option[Pipe] = {
    Pipe.values.find { pipeToTry =>
      pipeToTry.connections.forall { direction =>
        val spotInThatDirection = map.get(startingPosition + direction)
        spotInThatDirection.exists(pipeToTry.connectsTo(direction, _))
      }
    }
  }

  def replaceStarting(map: SpotsMap): (SpotsMap, Coords) = {
    val startingPosition = startingPositionCoords(map).getOrElse(throw new Exception("No starting position found"))
    val startingPipe =
      determinePipeTypeOfStartingPosition(map, startingPosition).getOrElse(throw new Exception("No starting pipe found"))
    val newMap = map.updated(startingPosition, Spot.OfPipe(startingPipe))
    (newMap, startingPosition)
  }

  def parseFully(lines: Vector[String]): (PipesMap, Coords) = {
    val (map, startingPosition) = parse(lines) |> toMap |> replaceStarting
    val pipesMap = map.iterator.collect { case (coords, Spot.OfPipe(pipe)) => coords -> pipe }.toMap
    (pipesMap, startingPosition)
  }

  def buildGraph(pipes: PipesMap): Graph[WithCoords[Pipe], UnDiEdge[WithCoords[Pipe]]] = {
    val edges = pipes.iterator.map(WithCoords.fromTuple).flatMap { withCoords =>
      withCoords.value.connectionsTo(withCoords.coords, pipes).map { case (coords, pipe) =>
        withCoords ~ WithCoords(coords, pipe)
      }
    }.toVector
    val graph = Graph.from(edges)
    graph
  }

  def run1(lines: Vector[String]): String = {
    val (map, startingPosition) = parseFully(lines)

    val graph = buildGraph(map)
    val startingNode = graph.get(WithCoords(startingPosition, map(startingPosition)))
    val cycle = graph.findCycleContaining(startingNode)(Visitor.empty).getOrElse(throw new Exception("No cycle found"))
    val cycleMap = cycle.collect { case n: graph.InnerNode => n.outer.asTuple }.toMap
    println(Bounds.render(
      cycleMap.view.mapValues(Spot.OfPipe.apply).toMap.updated(startingPosition, Spot.StartingPosition)
    ))

    (cycleMap.size / 2).toString
  }

  def run2(lines: Vector[String]): String = {
    val (pipesMap, startingPosition) = parseFully(lines)

    val graph = buildGraph(pipesMap)
    val startingNode = graph.get(WithCoords(startingPosition, pipesMap(startingPosition)))
    val cycle = graph.findCycleContaining(startingNode)(Visitor.empty).getOrElse(throw new Exception("No cycle found"))
    val cyclePoints = cycle.iterator.collect { case n: graph.InnerNode => n.outer }.toVector
    val cycleMap = cyclePoints.iterator.map(_.asTuple).toMap

    println(Bounds.render(cycleMap))

    val geometryFactory = new GeometryFactory()
    val polygon = geometryFactory.polygonOf(cyclePoints.iterator.map(_.coords))

    val bounds = Bounds.from(cycleMap.keysIterator)
    val groundCoords = bounds.coords.toSet -- cycleMap.keySet

    println(Bounds.render(groundCoords.iterator.map(_ -> '.').toMap))

    val innerGroundCoords = groundCoords.toVector.par.filter { groundCoord =>
      polygon.contains(geometryFactory.createPoint(groundCoord.asJTS))
    }
    val spotsMap = innerGroundCoords.foldLeft(
      cycleMap.view.mapValues(Spot.OfPipe.apply).toMap.updated(startingPosition, Spot.StartingPosition)
    )((map, groundCoord) => map.updated(groundCoord, Spot.Ground(isInner = true)))

    println("-----------")
    println(bounds.render(spotsMap))

    spotsMap.count(_._2.isInnerGround).toString
  }
}

object _10_1_Test1 extends Problem(10, InputMode.Test(1), Solution10.run1)
object _10_1_Test2 extends Problem(10, InputMode.Test(2), Solution10.run1)
object _10_1_Normal extends Problem(10, InputMode.Normal, Solution10.run1)

object _10_2_Test1 extends Problem(10, InputMode.Test(3), Solution10.run2)
object _10_2_Test2 extends Problem(10, InputMode.Test(4), Solution10.run2)
object _10_2_Test3 extends Problem(10, InputMode.Test(5), Solution10.run2)
object _10_2_Test4 extends Problem(10, InputMode.Test(6), Solution10.run2)
object _10_2_Normal extends Problem(10, InputMode.Normal, Solution10.run2)