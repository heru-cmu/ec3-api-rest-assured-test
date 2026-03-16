<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST User (Register)</name>
   <tag></tag>
   <elementGuidId>913f0667-4653-430f-8869-14f855d4db96</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;username\&quot;: \&quot;heruQA3\&quot;,\n  \&quot;firstName\&quot;: \&quot;Heru3\&quot;,\n  \&quot;lastName\&quot;: \&quot;Utama3\&quot;,\n  \&quot;email\&quot;: \&quot;heru3@test.com\&quot;,\n  \&quot;password\&quot;: \&quot;12345\&quot;,\n  \&quot;phone\&quot;: \&quot;08123456789\&quot;,\n  \&quot;userStatus\&quot;: 1\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0d5f1721-c421-4c68-9ecc-6a2d16872ebd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d185ad0a-4940-4cb4-b9dc-6764fea734af</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.0.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.configuration.RunConfiguration
import groovy.json.JsonSlurper

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// Status code check
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// Parse response
def slurper = new JsonSlurper()
def jsonResponse = slurper.parseText(response.getResponseBodyContent())

// Load schema file
def projectDir = RunConfiguration.getProjectDir()
def schemaFile = new File(projectDir + &quot;/Include/resources/schema/post-user-schema.json&quot;)
def schema = slurper.parseText(schemaFile.text)

// Validasi field sesuai schema
schema.properties.each { key, value ->
    assert jsonResponse.containsKey(key) : &quot;Field '${key}' tidak ada di response&quot;
    def actualType = jsonResponse[key]?.getClass()?.getSimpleName()?.toLowerCase()
    assert actualType.contains(value.type) : &quot;Field '${key}' tipe salah. Dapat: ${actualType}, Harus: ${value.type}&quot;
    println(&quot;Field '${key}' OK sebagai ${actualType}&quot;)
}

// Required field check
schema.required.each { req ->
    assert jsonResponse[req] != null : &quot;Field '${req}' wajib ada tapi null&quot;
}

println(&quot;✅ POST User response valid sesuai schema (strict)&quot;)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
