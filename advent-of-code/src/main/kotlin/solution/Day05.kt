package solution

import java.lang.Integer.parseInt
import java.util.*
import kotlin.collections.ArrayDeque

private class Operation (
    val move: Int,
    val from: Int,
    val to: Int,
) {
    companion object {
        fun fromStr(input: String): Operation {
            val re = """move (?<move>\d+) from (?<from>\d+) to (?<to>\d+)""".toRegex()
            val matches = re.matchEntire(input)?.groups ?: throw Exception("bad input format: $input")

            return Operation(move = parseInt(matches["move"]?.value),
                from = parseInt(matches["from"]?.value),
                to = parseInt(matches["to"]?.value))
        }
    }
}

class Day05 : Solution {


    data class Operation( val move: Int, val from: Int, val to: Int )

    private fun getOutput(stacks: Array<ArrayDeque<Char>>): String {
        var out = ""
        for (stack in stacks) {
            if (stack.isNotEmpty()) {
                out += stack.last()
            }
        }

        return out
    }


    private fun parseStacks(lines: List<String>): Pair<Array<ArrayDeque<Char>>, List<solution.Operation>> {
        val ops: MutableList<solution.Operation> = mutableListOf();

        var stacks: Array<ArrayDeque<Char>> = arrayOf()
        for (line in lines) {
            if (line.startsWith(" 1")) {
                stacks = Array(parseInt(line.toCharArray().last().toString())) { ArrayDeque() }
                break
            }
        }

        for (line in lines) {
            if (line.startsWith("move")) {
                ops.add(solution.Operation.fromStr(line))
            }
            if(!line.contains('[')) {
                continue
            }
            else {
                val chars = line.toCharArray()
                for (idx in 1..chars.size step 4) {
                    val nextChar = chars[idx]
                    if (nextChar != ' ') {
                        stacks[idx / 4].addFirst(nextChar)
                    }
                }
            }
        }

        return Pair(stacks, ops)
    }

    override fun solvePart1(input: String): String {
        val parsed = parseStacks(input.lines())
        val stacks = parsed.first
        val ops = parsed.second

        for (op in ops) {
            for (unused in 1..op.move) {
                if (stacks[op.from - 1].isNotEmpty()) {
                    stacks[op.to - 1].addLast(stacks[op.from - 1].removeLast())
                }
            }
        }

	return getOutput(stacks)
    }

    override fun solvePart2(input: String): String {
        val parsed = parseStacks(input.lines())
        val stacks = parsed.first
        val ops = parsed.second

        for (op in ops) {
            val toMove = ArrayDeque<Char>()
            for (u in 1..op.move) {
                if (stacks[op.from - 1].isNotEmpty()) {
                    toMove.addFirst(stacks[op.from - 1].removeLast())
                }
            }
            stacks[op.to - 1].addAll(toMove)
        }

        return getOutput(stacks)
    }

}
