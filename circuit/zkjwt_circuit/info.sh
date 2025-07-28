echo "Show the size of the ZK circuit..."
bb gates -b target/verified_anonymous_sns_jwt.json | grep "circuit"

# Logs:
#
# Scheme is: ultra_honk, num threads: 8
#         "circuit_size": 152379
