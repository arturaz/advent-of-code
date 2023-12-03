package aoc

object Solution3 {
  case class Part(no: Int) {
    lazy val noStr = no.toString

    /** Returns all coordinates of this part, starting from the given coordinate. */
    def allCoords(start: Coords): Iterator[Coords] = {
      (0 until noStr.length).iterator.map(i => start.right(i))
    }
  }

  case class SolutionMap(markers: Set[Coords], parts: Map[Coords, Part]) {
    lazy val bounds: Bounds = Bounds.from(markers.iterator ++ partsExpanded.keys)

    lazy val partsExpanded: Map[Coords, (Coords, Part)] = parts.flatMap { case (start, part) =>
      val coords = part.allCoords(start)
      coords.map(_ -> (start, part))
    }

    def render: String = {
      bounds.coordsByLine.map { line =>
        line.map { coord =>
          partsExpanded.get(coord) match {
            case None => if (markers.contains(coord)) 'X' else '.'
            case Some((start, part)) => part.noStr(coord.x - start.x)
          }
        }.mkString("")
      }.mkString("\n")
    }
  }

  def parseMap(data: Vector[String]): SolutionMap = {
    data.iterator.zipWithIndex
      .map { case (line, y) =>
        case class PartBuilder(startsAtIdx: Int, no: String) {
          def +(c: Char): PartBuilder = copy(no = no + c)
        }

        case class State(
          parts: Vector[PartBuilder], currentPart: Option[PartBuilder], markers: Set[Int]
        ) {
          def notDigitAt(idx: Int, c: Char): State = {
            val newState = currentPart match {
              case Some(part) => copy(parts = parts :+ part, currentPart = None)
              case None => this
            }
            c match {
              case '.' => newState
              case _ => newState.copy(markers = newState.markers + idx)
            }
          }

          def digitAt(idx: Int, c: Char): State = {
            currentPart match {
              case Some(part) => copy(currentPart = Some(part + c))
              case None => copy(currentPart = Some(PartBuilder(idx, c.toString)))
            }
          }
        }

        val state = line.iterator.zipWithIndex.foldLeft(
          State(Vector.empty, None, Set.empty)
        ) { case (state, (c, idx)) =>
          if (c.isDigit) state.digitAt(idx, c)
          else state.notDigitAt(idx, c)
        }.notDigitAt(line.length, '.')

        val markers = state.markers.map(x => Coords(x, y))
        val parts = state.parts.map { part =>
          val start = Coords(part.startsAtIdx, y)
          start -> Part(part.no.toInt)
        }

        (markers, parts)
      }
      .foldLeft(SolutionMap(Set.empty, Map.empty)) { case (map, (markers, parts)) =>
        map.copy(
          markers = map.markers ++ markers,
          parts = map.parts ++ parts
        )
      }
  }

  def run(data: Vector[String]): Unit = {
    val map = parseMap(data)
    println(map.render)

    val parts = map.markers.iterator.flatMap { marker =>
      marker.aroundWithDiagonals.flatMap(map.partsExpanded.get).distinct
    }.toVector
//    println(parts.mkString("\n"))
    val result = parts.iterator.map(_._2.no).sum

    println(result)
  }
}

object _3_1_Test extends Problem(3, InputMode.Test(1), Solution3.run)
object _3_1_Normal extends Problem(3, InputMode.Normal, Solution3.run)