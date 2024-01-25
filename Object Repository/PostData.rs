<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostData</name>
   <tag></tag>
   <elementGuidId>55f52a08-2ee7-4bbb-8c77-d50492df184d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${Ltoken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;lob\&quot; : \&quot;MMU\&quot;,\n    \&quot;fullName\&quot; : \&quot;MARIYONO\&quot;,\n    \&quot;mobilePhone\&quot; : \&quot;081585598942\&quot;,\n    \&quot;noKtp\&quot; : \&quot;3173082011800008\&quot;,\n    \&quot;birthDate\&quot; : \&quot;1980-11-20\&quot;,\n    \&quot;birthPlace\&quot; : \&quot;JAKARTA\&quot;,\n    \&quot;gender\&quot; : \&quot;M\&quot;,\n    \&quot;mothersName\&quot; : \&quot;RODIAH\&quot;,\n    \&quot;financingType\&quot; : \&quot;U\&quot;,\n    \&quot;packageCode\&quot; : \&quot;PR0002\&quot;,\n    \&quot;tenor\&quot; : 12,\n    \&quot;downPayment\&quot; : 500000,\n    \&quot;channel\&quot; : \&quot;FNA\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>88bdf418-c07d-4204-b93d-53cbe0fad712</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Ltoken}</value>
      <webElementGuid>ae54bcee-fc26-4f67-8890-f504c039525e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/leads/draft</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>2309fc43-4c92-418b-82e3-1e2c6ce2be3d</id>
      <masked>false</masked>
      <name>Ltoken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.console.ui.SystemOutputInterceptor
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//GlobalVariable.token = WS.getElementPropertyValue(response, 'result.accessToken')
System.out.println(GlobalVariable.token)

GlobalVariable.getdraftid = WS.getElementPropertyValue(response, 'result.draftId')
System.out.println(GlobalVariable.getdraftid)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
