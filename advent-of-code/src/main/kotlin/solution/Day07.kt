package solution

import java.lang.Exception
import java.lang.Integer.parseInt
import java.lang.NullPointerException

data class File (
    val size: Int,
    val name: String
)

class Dir constructor(val name: String, val parent: Dir?, var files: ArrayDeque<File> = ArrayDeque(), val subDirs: ArrayDeque<Dir> = ArrayDeque()) {
    fun size(): Int {
        var total = 0
        total += files.map(File::size).sum()
        total += subDirs.map(Dir::size).sum()

        return total
    }

}

fun parseCommands(lines: List<String>): Dir {
    val root = Dir("/", null)
    var parent = root
    var head = root

    for (line in lines) {
        val cmdReg = "(\\\$) ([a-z]+) *(.*)".toRegex()
        if (line.matches(cmdReg)) {
            val (_, cmd, cmdVal) = cmdReg.matchEntire(line)!!.destructured
            when(cmd) {
                "cd" -> head = when (cmdVal) {
                    "/" -> root
                    ".." -> head.parent ?:
                        throw NullPointerException("No parent of current dir")
                    else -> head.subDirs.find { it.name == cmdVal } ?:
                        throw NullPointerException("Dir not found in current")
                }
                "ls" -> continue
                else -> throw Exception("Unknown cmd: $cmd")
            }
        }
        else {
            val dirReg = "(dir) ([a-z]+)".toRegex()
            val fileReg = "([0-9]+) ([a-z]+\\.*[a-z]*)".toRegex()
            when {
                line.matches(dirReg) -> {
                    val (_, dirname) = dirReg.matchEntire(line)!!.destructured
                    head.subDirs.addLast(Dir(dirname, head))
                }

                line.matches(fileReg) -> {
                    val (size, filename) = fileReg.matchEntire(line)!!.destructured
                    head.files.addLast(File(parseInt(size), filename))
                }

                else -> throw Exception("Unknown line format")
            }
        }
    }

    return root
}


class Day07 : Solution {

    override fun solvePart1(input: String): String {

        val root: Dir = parseCommands(input.lines())

        var total = 0

        val queue: ArrayDeque<Dir> = ArrayDeque()
        queue.addFirst(root)
        while(queue.isNotEmpty()) {
            val next = queue.removeFirst()
            queue.addAll(next.subDirs)

            val size = next.size()
            if (size <= 100000) {
               total += size
            }
        }

        return total.toString()
    }

    override fun solvePart2(input: String): String {

        val diskSize = 70000000
        val targetUnused = 30000000

        val root: Dir = parseCommands(input.lines())

        var usedSpace = root.size()
        var unusedSpace = diskSize - usedSpace

        val optionDirs: MutableList<Dir> = mutableListOf()
        var queue: ArrayDeque<Dir> = ArrayDeque()
        queue.addFirst(root)

        while(queue.isNotEmpty()) {
            val next = queue.removeFirst()
            queue.addAll(next.subDirs)

            val dirSize = next.size()
            if (unusedSpace + dirSize >= targetUnused) {
                optionDirs.add(next)
            }
        }


        return optionDirs.map(Dir::size).minOrNull()!!.toString()
    }

}
