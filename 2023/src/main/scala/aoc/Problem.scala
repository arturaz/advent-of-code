package aoc

import scala.util.Using

enum InputMode {
  case Test(subProblemNo: Int)
  case Normal
}

trait Problem(problemNo: Int, inputMode: InputMode, run: Vector[String] => Unit) {
  def name: String = {
    val fileName = inputMode match {
      case InputMode.Test(subProblemNo) => s"${problemNo}_${subProblemNo}_test"
      case InputMode.Normal => s"${problemNo}_normal"
    }
    
    s"data/$fileName.txt"
  }
  
  def main(args: Array[String]): Unit = {
    val data = Using(io.Source.fromFile(name)) { source =>
      source.getLines().toVector
    }.get
    run(data)
  }
}
