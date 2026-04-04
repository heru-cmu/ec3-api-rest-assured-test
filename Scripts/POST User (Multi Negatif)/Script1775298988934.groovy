import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper

// Daftar skenario negatif
def negativeCases = [
    [desc: "Username kosong", body: '''{ "id":123,"username":"","firstName":"Neg","lastName":"Case","email":"valid@test.com","password":"12345","phone":"08123456789","userStatus":1 }'''],
    [desc: "Email invalid", body: '''{ "id":123,"username":"heruQA3","firstName":"Neg","lastName":"Case","email":"invalidEmail","password":"12345","phone":"08123456789","userStatus":1 }'''],
    [desc: "Phone bukan angka", body: '''{ "id":123,"username":"heruQA3","firstName":"Neg","lastName":"Case","email":"valid@test.com","password":"12345","phone":"abc123","userStatus":1 }'''],
    [desc: "ID string", body: '''{ "id":"abc","username":"heruQA3","firstName":"Neg","lastName":"Case","email":"valid@test.com","password":"12345","phone":"08123456789","userStatus":1 }''']
]

def results = []

negativeCases.each { testCase ->
    println("▶️ Testing Negative Case: ${testCase.desc}")

    RequestObject request = findTestObject('Petstore/POST User (Register)')
    request.setBodyContent(new com.kms.katalon.core.testobject.impl.HttpTextBodyContent(testCase.body, "UTF-8", "application/json"))

    ResponseObject response = WS.sendRequest(request)
    def jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())

    // Evaluasi hasil
    if (response.getStatusCode() == 200) {
        println("⚠️ BUG: API menerima input invalid → ${testCase.desc}")
        results << "BUG → ${testCase.desc}"
    } else {
        println("✅ API menolak input invalid → ${testCase.desc} (status: ${response.getStatusCode()})")
        results << "OK → ${testCase.desc} (status: ${response.getStatusCode()})"
    }

    println("Response body: " + jsonResponse)
    println("--------------------------------------------------")
}

// Summary akhir
println("===== SUMMARY NEGATIVE POST USER =====")
results.each { println(it) }
