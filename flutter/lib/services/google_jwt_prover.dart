import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:typed_data';
import '../models/signed_message.dart';
import 'package:mopro_flutter/mopro_flutter.dart';
import 'package:mopro_flutter/mopro_flutter_platform_interface.dart';

import 'generate_ephemeral_key.dart';

Future<Uint8List?> generateJwtProof(
  String jwt,
  String? idToken,
  String domain,
) async {
  if (idToken == null) {
    throw Exception('ID Token is null');
  }
  final moproFlutterPlugin = MoproFlutter();
  const srsAssetPath = 'assets/jwt-srs.local';
  final srsPath = await moproFlutterPlugin.copyAssetToFileSystem(srsAssetPath);
  final ephemeralKey = await getEphemeralKey();

  // Decode the JSON string
  Map<String, dynamic> ephemeralKeyObj = jsonDecode(ephemeralKey);
  final ephemeralPubkey = ephemeralKeyObj['public_key'];
  final ephemeralSalt = ephemeralKeyObj['salt'];
  final ephemeralExpiry = ephemeralKeyObj['expiry'];

  try {
    final proof = await moproFlutterPlugin.proveJwt(
      srsPath,
      ephemeralPubkey,
      ephemeralSalt,
      ephemeralExpiry,
      idToken,
      jwt,
      domain,
    );

    if (proof == null) {
      throw Exception('Proof is null: ${proof?.error}');
    } else {
      return proof.proof;
    }
  } catch (e) {
    print('Error creating message: $e');
  }
}
