<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Token_Fiona_FIF</name>
   <tag></tag>
   <elementGuidId>76432c49-cb78-4218-87de-a145111b5274</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userId\&quot;: \&quot;${user}\&quot;,\n    \&quot;password\&quot;: \&quot;${pass}\&quot;\n}\n&quot;,
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
      <webElementGuid>fca3efb8-5467-40c4-a867-2466635b44ed</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/user/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'fiona'</defaultValue>
      <description>fiona</description>
      <id>235c7583-083f-4cb0-8cb9-f1e7418400e9</id>
      <masked>false</masked>
      <name>user</name>
   </variables>
   <variables>
      <defaultValue>'zQe9Rk4qqvzs3Xyz'</defaultValue>
      <description>zQe9Rk4qqvzs3Xyz</description>
      <id>856509cf-765d-4030-8521-ca7ae2e97f70</id>
      <masked>false</masked>
      <name>pass</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.token = WS.getElementPropertyValue(response, 'result.accessToken')
System.out.println (GlobalVariable.token)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
