import Flutter
import UIKit
import moproFFI

public class MoproFlutterPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(
      name: "mopro_flutter", binaryMessenger: registrar.messenger())
    let instance = MoproFlutterPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    // Run potentially long-running tasks on a background thread
    DispatchQueue.global(qos: .userInitiated).async {
      let dispatchResult: (Any?) -> Void = { response in
        DispatchQueue.main.async {
          result(response)
        }
      }
      let dispatchError: (String, String?, Any?) -> Void = { code, message, details in
        DispatchQueue.main.async {
          result(FlutterError(code: code, message: message, details: details))
        }
      }

      switch call.method {
      case "getPlatformVersion":
        dispatchResult("iOS " + UIDevice.current.systemVersion)
      case "getApplicationDocumentsDirectory":
        do {
          let documentsPath = try FileManager.default.url(
            for: .documentDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: false
          ).path
          dispatchResult(documentsPath)
        } catch {
          dispatchError(
            "DIR_ERROR", "Could not get documents directory", error.localizedDescription)
        }
      case "proveJwt":
        guard let args = call.arguments as? [String: Any],
          let srsPath = args["srsPath"] as? String,
          let ephemeralPublicKey = args["ephemeralPublicKey"] as? String,
          let ephemeralSalt = args["ephemeralSalt"] as? String,
          let ephemeralExpiry = args["ephemeralExpiry"] as? String,
          let tokenId = args["tokenId"] as? String,
          let jwt = args["jwt"] as? String,
          let domain = args["domain"] as? String
        else {
          dispatchError(
            "INVALID_ARGUMENTS",
            "srsPath, ephemeralPublicKey, ephemeralSalt, ephemeralExpiry, tokenId, jwt, domain are null or invalid format",
            nil)
          return
        }
        do {
          // Note: The mopro.swift binding `proveZkemail` doesn't throw currently,
          // but we include a do-catch for future-proofing and consistency.
          let proofBytes = proveJwt(
            srsPath: srsPath,
            ephemeralPubkey: ephemeralPublicKey,
            ephemeralSalt: ephemeralSalt,
            ephemeralExpiry: ephemeralExpiry,
            tokenId: tokenId,
            jwt: jwt,
            domain: domain
          )
          // Wrap the result in the format expected by Dart
          let resultMap: [String: Any?] = [
            "proof": FlutterStandardTypedData(bytes: proofBytes), "error": nil,
          ]
          dispatchResult(resultMap)
        } catch {
          // If proveZkemail were to throw, handle it here
          let resultMap: [String: Any?] = ["proof": nil, "error": error.localizedDescription]
          dispatchResult(resultMap)
        }
      case "verifyJwtProof":
        guard let args = call.arguments as? [String: Any],
          let srsPath = args["srsPath"] as? String,
          let proofData = args["proof"] as? FlutterStandardTypedData,
          let domain = args["domain"] as? String,
          let googleJwtPubkeyModulus = args["googleJwtPubkeyModulus"] as? String,
          let ephemeralPubkey = args["ephemeralPubkey"] as? String,
          let ephemeralPubkeyExpiry = args["ephemeralPubkeyExpiry"] as? String
        else {
          dispatchError("INVALID_ARGUMENTS", "srsPath or proof is null or invalid format", nil)
          return
        }
        do {
          let isValid = verifyJwtProof(
            srsPath: srsPath, proof: proofData.data, domain: domain,
            googleJwtPubkeyModulus: googleJwtPubkeyModulus, ephemeralPubkey: ephemeralPubkey,
            ephemeralPubkeyExpiry: ephemeralPubkeyExpiry)
          let resultMap: [String: Any?] = ["isValid": isValid, "error": nil]
          dispatchResult(resultMap)
        } catch {
          let resultMap: [String: Any?] = ["isValid": false, "error": error.localizedDescription]
          dispatchResult(resultMap)
        }
      case "signMessage":
        guard let args = call.arguments as? [String: Any],
          let anonGroupId = args["anonGroupId"] as? String,
          let text = args["text"] as? String,
          let isInternal = args["internal"] as? Bool,
          let ephemeralPublicKey = args["ephemeralPublicKey"] as? String,
          let ephemeralPrivateKey = args["ephemeralPrivateKey"] as? String,
          let ephemeralPubkeyExpiry = args["ephemeralPubkeyExpiry"] as? String
        else {
          dispatchError(
            "INVALID_ARGUMENTS",
            "anonGroupId or text or internal or ephemeralPubkey or ephemeralPrivateKey or ephemeralPubkeyExpiry is null or invalid format",
            nil)
          return
        }
        do {
          let signedMessage = signMessage(
            anonGroupId: anonGroupId, text: text, internal: isInternal,
            ephemeralPublicKey: ephemeralPublicKey, ephemeralPrivateKey: ephemeralPrivateKey,
            ephemeralPubkeyExpiry: ephemeralPubkeyExpiry)
          dispatchResult(signedMessage)
        } catch {
          dispatchError("SIGN_MESSAGE_ERROR", "Error signing message", error.localizedDescription)
        }
      case "generateEphemeralKey":
        do {
          let ephemeralKey = generateEphemeralKey()
          dispatchResult(ephemeralKey)
        } catch {
          dispatchError(
            "GENERATE_EPHEMERAL_KEY_ERROR", "Error generating ephemeral key",
            error.localizedDescription)
        }
      default:
        dispatchResult(FlutterMethodNotImplemented)
      }
    }  // End of DispatchQueue.global().async
  }
}
