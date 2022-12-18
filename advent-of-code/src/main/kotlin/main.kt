import solution.Day05

fun getResourceAsText(path: String): String {
    val resource = object {}.javaClass.getResource(path) ?: throw Exception("resource not found: $path")
    return resource.readText()
}

fun main(args: Array<String>) {

}
