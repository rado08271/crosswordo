package common.utils

import common.constant.Dictionary
import common.constant.Settings
import entity.board.Word
import kotlin.collections.HashMap

class DictionaryUtils {
    fun toMap(lines: List<String>) : MutableMap<String, Int> {

        val immutableMap: MutableMap<String, Int> = HashMap()

        lines.reversed().subList(0, lines.size - 2)
            .map { it.split("\t") }
            .filter {
                it.size == 2 &&
                it[0].isNotEmpty() &&
                it[0].length >= 3 &&
                it[1].toIntOrNull() != null &&
                it[1].toInt() > 100
            }
            .forEach() {
                immutableMap.put(it[0].lowercase(), it[1].toInt())
            }

        return immutableMap
    }

    fun toMap(): MutableMap<String, Int> {
        val immutableMap: MutableMap<String, Int> = HashMap()

        Dictionary.orig.split("\n").map {
            val entity = it.split("\t")
            // do not lead word longer than MAX or shorter than 3
            if (entity[0].length > 2 && entity[0].length <= Settings.MAX)
                immutableMap.put(entity[0].lowercase(), entity[1].toInt())
        }

        return immutableMap
    }

    // this will be used for searching data - map is 0N instead of 0logn
    fun toMapLength(wordLength: Int): MutableMap<String, Int> {
        val immutableMap: MutableMap<String, Int> = HashMap()

        var tokenized = Dictionary.orig.split("\n").map {
            val entity = it.split("\t")
            // do not lead word longer than MAX or shorter than 3
            if (entity[0].length > 3 && entity[0].length <= Settings.MAX)
                if (entity[0].length == wordLength)
                    immutableMap.put(entity[0], entity[1].toInt())
        }

        return immutableMap
    }
}