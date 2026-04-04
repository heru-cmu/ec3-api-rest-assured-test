import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

// Jalankan POST Positif
WebUI.callTestCase(findTestCase('POST User (Positif)'), [:], FailureHandling.STOP_ON_FAILURE)

// Jalankan POST Multi Negatif
WebUI.callTestCase(findTestCase('POST User (Multi Negatif)'), [:], FailureHandling.CONTINUE_ON_FAILURE)

// Jalankan PUT Positif
WebUI.callTestCase(findTestCase('PUT User (Positif)'), [:], FailureHandling.STOP_ON_FAILURE)

// Jalankan PUT Multi Negatif
WebUI.callTestCase(findTestCase('PUT User (Multi Negatif)'), [:], FailureHandling.CONTINUE_ON_FAILURE)

// Jalankan GET Positif
WebUI.callTestCase(findTestCase('GET User (Positif)'), [:], FailureHandling.STOP_ON_FAILURE)

// Jalankan GET Multi Negatif
WebUI.callTestCase(findTestCase('GET User (Multi Negatif)'), [:], FailureHandling.CONTINUE_ON_FAILURE)

// Jalankan DELETE Positif
WebUI.callTestCase(findTestCase('DELETE User (Positif)'), [:], FailureHandling.STOP_ON_FAILURE)

// Jalankan DELETE Multi Negatif
WebUI.callTestCase(findTestCase('DELETE User (Multi Negatif)'), [:], FailureHandling.CONTINUE_ON_FAILURE)
