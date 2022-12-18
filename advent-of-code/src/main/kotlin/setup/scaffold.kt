package setup

import java.io.File
import java.lang.Integer.parseInt
import java.nio.file.Files
import java.nio.file.Paths

fun main(args: Array<String>) {
    val day = parseInt(args[0])
    val dayPadded = day.toString().padStart(2, '0')

    val solutionTemplate =
        """
            package solution

            class Day${dayPadded} : Solution {

                override fun solvePart1(input: String): String {
                    TODO("Not yet implemented")
                }

                override fun solvePart2(input: String): String {
                    TODO("Not yet implemented")
                }

            }
        """.trimIndent()

    val solutionOutput = Paths.get("src/main/kotlin/solution/Day$dayPadded.kt")
    if (!Files.exists(solutionOutput)) {
        solutionOutput.toFile().writeText(solutionTemplate)
    }

    val testTemplate =
        """
            import org.testng.annotations.Test
            import solution.Day${dayPadded}
            import kotlin.test.assertEquals

            class Day${dayPadded}Test {

                @Test
                fun testDay${dayPadded}_part1() {
                    val input = Util.getResourceAsText("examples/${dayPadded}.txt")

                    val day = Day${dayPadded}()
                    val result = day.solvePart1(input)

                    assertEquals(0, result)
                }

                @Test
                fun testDay${dayPadded}_part2() {
                    val input = Util.getResourceAsText("examples/${dayPadded}.txt")

                    val day = Day${dayPadded}()
                    val result = day.solvePart2(input)

                    assertEquals(0, result)
                }
            }
        """.trimIndent()

    val testOutput = Paths.get("src/test/kotlin/solution/Day${dayPadded}Test.kt")
    if (!Files.exists(testOutput)){
        testOutput.toFile().writeText(testTemplate)
    }

    val examplePath = Paths.get("src/test/resources/examples/${dayPadded}.txt")
    if (!Files.exists(examplePath)) {
        Files.createFile(examplePath)
    }
}
