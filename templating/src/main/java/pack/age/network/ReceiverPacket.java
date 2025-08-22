package ``MOD_GROUP``.network;

import net.minecraft.network.packet.CustomPayload;

public interface ReceiverPacket<C> extends CustomPayload {
    void receive(C context);
}
