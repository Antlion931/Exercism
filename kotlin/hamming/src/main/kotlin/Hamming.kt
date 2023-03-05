object Hamming {

    fun compute(leftStrand: String, rightStrand: String): Int {
        if (leftStrand.length != rightStrand.length) {
            throw IllegalArgumentException("left and right strands must be of equal length")
        }

        var result = 0

        for (i in 0 until leftStrand.length) {
            if (leftStrand[i] != rightStrand[i]) {
                result += 1;
            }
        }
        
        return result
    }
}
