import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import java.net.URLEncoder

def negativeCases = [
    [desc: "User tidak ada", username: "userTidakAda123"],
    [desc: "Username kosong", username: ""],
    [desc: "Username special char", username: "@@@###"]
]

def results = []

negativeCases.each { testCase ->
    println("▶️ Testing GET Case: ${testCase.desc}")

    RequestObject request = findTestObject('Petstore/GET User')

    // Encode username supaya karakter khusus aman di URL
    def encodedUsername = URLEncoder.encode(testCase.username, "UTF-8")
    request.setRestUrl("https://petstore.swagger.io/v2/user/${encodedUsername}")

    ResponseObject response = WS.sendRequest(request)

    def jsonResponse
    try {
        jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())
    } catch(Exception e) {
        jsonResponse = [error: "Tidak bisa parse body"]
    }

    // Evaluasi hasil
    if (response.getStatusCode() == 200) {
        println("⚠️ BUG: API menerima GET invalid → ${testCase.desc}")
        results << "BUG → ${testCase.desc}"
    } else {
        println("✅ API menolak GET invalid → ${testCase.desc} (status: ${response.getStatusCode()})")
        results << "OK → ${testCase.desc} (status: ${response.getStatusCode()})"
    }

    println("Response body: " + jsonResponse)
    println("--------------------------------------------------")
}

// Summary akhir
println("===== SUMMARY NEGATIVE GET USER =====")
results.each { println(it) }
